
// Problem statement: https://holochain-gym.github.io/developers/basic/zome-functions/
use hdk::prelude::*;
use hdk::info::agent_info;

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeExternalInput {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeExternalOutput(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Greeting(String);

#[hdk_extern]
pub fn hello_world(_:()) -> ExternResult<SomeExternalOutput> {
    let message: String = String::from("Hello world");
    let output: SomeExternalOutput = SomeExternalOutput(message);
    
    Ok(output)
}

#[hdk_extern]
pub fn goodbye_world(_:()) -> ExternResult<SomeExternalOutput> {
    Ok(SomeExternalOutput(String::from("goodbye")))
}

#[hdk_extern]
pub fn say_my_name(external_input:SomeExternalInput) -> ExternResult<SomeExternalOutput> {
    let message: String = format!("Your name is {} {}", 
                                    external_input.first_name, 
                                    external_input.last_name);
    let output: SomeExternalOutput = SomeExternalOutput(message);
    
    Ok(output)
}

#[hdk_extern]
pub fn say_greeting(external_input:Greeting) -> ExternResult<SomeExternalOutput> {
    let greet: String = external_input.0;
    let output = SomeExternalOutput(format!("I greet you, {}", greet));
    Ok(output)
}

#[hdk_extern]
pub fn get_agent_id(_:()) -> ExternResult<AgentInfo> {
    Ok(agent_info()?)
}