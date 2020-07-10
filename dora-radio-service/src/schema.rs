use juniper::FieldResult;
use crate::model::Subsystem;

type Context = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

graphql_object!(QueryRoot: Context as "Query" |&self| {
    // Test query to verify service is running without attempting
    // to communicate with the underlying subsystem
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }

    // Test query to very arbitrary transfers through the radio are working
    field echo(&executor, data: Option<String>) -> FieldResult<String>
    {
        match data {
            Some(d) => Ok(d),
            None =>  Ok("empty data field".to_owned())
        }
    }

    // Returns the file at the specified (full) path, optionally encoded in base64  
    field run_command(&executor, 
        path: Option<String>, args: Option<Vec<String>>,
        stdout: Option<String>, stderr: Option<String>) -> FieldResult<String>
    {
        Ok(executor.context().subsystem().run_command(path, args, stdout, stderr)?)
    }

    // Returns the file at the specified (full) path, optionally encoded in base64  
    field download_file(&executor, path: Option<String>, encode: Option<bool>) -> FieldResult<String>
    {
        Ok(executor.context().subsystem().download_file(path, encode)?)
    }

    // Uploads the contents in data to a file at the specified path, 
    // optionally decodes data from base64 before writing to file
    field upload_file(&executor, path: Option<String>, decode: Option<bool>, data: Option<String>) -> FieldResult<String>
    {
        Ok(executor.context().subsystem().upload_file(path, decode, data)?)
    }    

    // Request number of bad uplink packets
    field failed_packets_up(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().failed_packets_up()?)
    }

    // Request number of bad downlink packets
    field failed_packets_down(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().failed_packets_down()?)
    }

    // Request number of packets successfully uplinked
    field packets_up(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().packets_up()?)
    }

    // Request number of packets successfully downlinked
    field packets_down(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().packets_down()?)
    }

    // Request errors that have occured
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        Ok(executor.context().subsystem().errors()?)
    }
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {

});