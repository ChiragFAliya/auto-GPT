use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::fs;

pub const CODE_TEMPLATE_PATH: &str = "/Users/chira/Desktop/auto-gpt/web_templet/src/code_template.rs";

pub const WEB_SERVER_PROJECT_PATH: &str = "/Users/chira/Desktop/auto-gpt/web_templet/";

pub const EXEC_MAIN_PATH: &str = "/Users/chira/Desktop/auto-gpt/web_templet/src/main.rs";

pub const API_SCHEMA_PATH: &str = "/Users/chira/Desktop/auto-gpt/auto_gippity/schemas/api_schema.json";



pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str: &str = ai_func(func_input);

    

    let msg: String = format!("FUNCTION: {} INSTRUCTION: you are a function priter.you ONLY print the result of functions.
    Nothing else.No commentery.Here is the input to the function: {}.
    Print out what the function will return.",
    ai_function_str, func_input);

    

    Message {
        role: "system".to_string(),
        content: msg,
    }
}



pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    
    let llm_respons_res = call_gpt(vec![extended_msg.clone()]).await;

    
    match llm_respons_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("failed twice to connect OpenAI"),
    }
}



pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(&llm_response.as_str())
        .expect("failed to decode ai response from json");

    return decoded_response;
}


pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}


pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("failed to read file contents!")
}

pub fn read_exec_main_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("failed to read file contents!")
}


pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("failed to write main.rs file");
}


pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("failed to write api endpoints to file");
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]

    async fn tests_ai_task_request() {
        let ai_func_param = "build me web server for making stock price api requests.".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining User requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
