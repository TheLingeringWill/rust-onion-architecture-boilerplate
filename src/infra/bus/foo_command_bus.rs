use crate::app::command::FooCommand;
use super::sqs_command_bus::SqsCommandBus;

pub type FooCommandBus = SqsCommandBus<FooCommand>;
