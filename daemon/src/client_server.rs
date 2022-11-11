use client_daemon::client_daemon_server::ClientDaemon;
use client_daemon::{
    ChatHistoryRequest, ChatHistoryResponse, DeleteMessageRequest, DeleteMessageResponse,
    EditMessageRequest, EditMessageResponse, Event, SendMessageRequest, SendMessageResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod client_daemon {
    tonic::include_proto!("clientdaemon");
}

#[derive(Debug)]
// FIXME: name this something better
pub struct ClientDaemonService {}

#[tonic::async_trait]
impl ClientDaemon for ClientDaemonService {
    async fn get_chat_history(
        &self,
        request: Request<ChatHistoryRequest>,
    ) -> Result<Response<ChatHistoryResponse>, Status> {
        println!("Got chat history request: {:?}", request);

        // TODO: Implement returning chat history with database, etc.
        return Ok(Response::new(ChatHistoryResponse::default()));
    }

    type SubscribeToEventsStream = ReceiverStream<Result<Event, Status>>;

    async fn subscribe_to_events(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::SubscribeToEventsStream>, Status> {
        println!("Received subscribe to events request: {:?}", request);
        // FIXME: Implement this
        let (_tx, rx) = tokio::sync::mpsc::channel(4);

        // TODO: send events as they come
        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        println!("Received send message request: {:?}", request);

        // TODO: implement sending messages
        return Ok(Response::new(SendMessageResponse::default()));
    }

    async fn edit_message(
        &self,
        request: Request<EditMessageRequest>,
    ) -> Result<Response<EditMessageResponse>, Status> {
        println!("Received edit message request: {:?}", request);

        // TODO: implement editing messages
        return Ok(Response::new(EditMessageResponse::default()));
    }

    async fn delete_message(
        &self,
        request: Request<DeleteMessageRequest>,
    ) -> Result<Response<DeleteMessageResponse>, Status> {
        println!("Received delete message request: {:?}", request);

        // TODO: implement deleting messages
        return Ok(Response::new(DeleteMessageResponse::default()));
    }
}
