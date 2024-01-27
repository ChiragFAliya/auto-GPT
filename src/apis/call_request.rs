use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{Client, Url};
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

//call large language model

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String = env::var("OPEN_AI_KEY").unwrap();
    let api_org: String =
        env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in the environment variables.");

    //confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    //create headers

    let mut headers = HeaderMap::new();

    //create api key header

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    //create openai org header

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    //create chat completion

    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // //troubleshooting

    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();

    // dbg!(res_raw.text().await.unwrap());

    //Extract API Response

    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // print!("hiii   {:?}",res);

    //send response

    // Ok(res.choices[0].message.content.clone())
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[tokio::test]

    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "hi there, this is a test. give me a short answer.".to_string(),
        };

        let messages = vec![message];

        let res = call_gpt(messages).await;
        if let Ok(res_str) = res {
            dbg!(res_str);
            assert!(true)
        } else {
            print!("{}", res.unwrap());
            assert!(false)
        }
    }
}
