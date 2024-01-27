use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    check_status_code, read_code_template_contents, save_api_endpoints, save_backend_code,
    read_exec_main_contents, WEB_SERVER_PROJECT_PATH
};

use crate::helpers::command_line::{PrintCommand, confirm_safe_code};
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, RouteObject, SpecialFunctions};

use async_trait::async_trait;
use reqwest::{Client, StatusCode, Url};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::vec;
use tokio::time;

#[derive(Debug)]

pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Developes backend  code for web server and database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

        let msg_context: String = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet,
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = read_code_template_contents();

        let msg_context: String = format!(
            "BROKEN CODE: {:?} \n ERROR_BUGS: {:?} \n 
    THIS FUNCTION ONLY OUTPUTS THE CODE . JUST OUTPUT THE CODE",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String{
        let backend_code: String = read_exec_main_contents();

        let msg_context: String = format!("CODE INPUT {}",backend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        ai_response 
    }

}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agents(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            
            match  &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                        self.attributes.state = AgentState::UnitTesting;
                        continue;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                        self.attributes.state = AgentState::UnitTesting;
                        continue;
                    }
                }
                AgentState::UnitTesting => {

                    PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), 
                    "backend code unit testing: Requesting user input");

                    let is_safe_code = confirm_safe_code();

                    if !is_safe_code {
                        panic!("Better go work on AI alignment instead......    ")
                    }

                    PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), 
                    "backend code unit testing: Building project");

                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build").current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to build backend application");

                    if build_backend_server.status.success() {
                        self.bug_count = 0;

                        PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), 
                        "backend code unit testing: Test server build successful...");
                    } else {
                        let error_array = build_backend_server.stderr;
                        let error_str = String::from_utf8(error_array).unwrap();

                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);

                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(), 
                            "backend code unit testing: Too may bugs found in code");

                            panic!("Error ! Too may bugs")
                        }

                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    let api_endpoint_str = self.call_extract_rest_api_endpoints().await;

                    let api_endpoints: Vec<RouteObject> = serde_json::from_str(&api_endpoint_str.as_str())
                        .expect("Failed to decode API Endpoints");

                    let check_endpoints:Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object|{
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        }).cloned().collect();

                        factsheet.api_endpoint_schema = Some(check_endpoints.clone());

                        PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), 
                        "backend code unit testing: Starting web server...");

                        let mut run_backend_server: std::process::Child = Command::new("cargo")
                        .arg("run").current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend application");


                        PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), 
                        "backend code unit testing: Launching test on server in 4 seconds...");

                        let second_sleep: Duration = Duration::from_secs(5);
                        time::sleep(second_sleep).await;

                        for endpoint in check_endpoints {

                            let testing_msg = format!("Testing endpoint '{}'...", endpoint.route);
                            PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(),
                            testing_msg.as_str()
                            );

                            let client: Client = Client::builder()
                                .timeout(Duration::from_secs(5))
                                .build()
                                .unwrap();

                            let url = format!("http://localhost:8080{}", endpoint.route);

                            match check_status_code(&client, &url).await {
                                Ok(status_code) => {
                                    if status_code != 200 {
                                        let error_msg = format!("WARNING ! Failed to call backend url endpoint {}",endpoint.route);
                                        PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(),
                                        error_msg.as_str()
                                        );
            
                                    }
                                }

                                Err(e) => {

                                    run_backend_server
                                        .kill()
                                        .expect("Failed to kill backend server");

                                        

                                        let error_msg = format!("Error checking backend {}", e);
                                        PrintCommand::Issue.print_agent_message(self.attributes.position.as_str(),
                                        error_msg.as_str()
                                        );
                                
                                }
                            }
                            
                        }


                    save_api_endpoints(&api_endpoint_str);
                    PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(),
                    "Backend testing is complete..."
                    );

                    run_backend_server
                                        .kill()
                                        .expect("Failed to kill backend server on completion");


                    self.attributes.state = AgentState::Finished;

                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_backend_developer() {
        let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

        let factsheet_str: &str = r#"
      {
        "project_description": "build a website that fetches and tracks fitness progress with timezone information",
        "project_scope": {
          "is_crud_required": true,
          "is_user_login_and_logout": true,
          "is_external_urls_required": true
        },
        "external_urls": [
          "http://worldtimeapi.org/api/timezone"
        ],
        "backend_code": null,
        "api_endpoint_schema": null
      }"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent.attributes.state = AgentState::Discovery;

        
        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer agent");
    }
}