use client_daemon::client_daemon_server::ClientDaemon;
use client_daemon::{
    ChatHistoryRequest, ChatHistoryResponse, DeleteMessageRequest, DeleteMessageResponse,
    EditMessageRequest, EditMessageResponse, Event, SendMessageRequest, SendMessageResponse,
    UiStateResponse,
};
use std::pin::Pin;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};

pub mod client_daemon {
    tonic::include_proto!("clientdaemon");
}

#[derive(Debug)]
// FIXME: name this something better
pub struct ClientDaemonService {
    pub ev_stream: tokio::sync::broadcast::Sender<Result<Event, Status>>,
}

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

    async fn get_ui_state(
        &self,
        request: Request<()>,
    ) -> Result<Response<UiStateResponse>, Status> {
        println!("Got UI state request: {:?}", request);

        // TODO: implement
        Ok(Response::new(UiStateResponse::default()))
    }

    type SubscribeToEventsStream =
        Pin<Box<dyn Stream<Item = Result<Event, Status>> + Send + Sync + 'static>>;

    async fn subscribe_to_events(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::SubscribeToEventsStream>, Status> {
        println!("Received subscribe to events request: {:?}", request);
        // FIXME: Implement this
        //let (_tx, rx) = tokio::sync::mpsc::channel(4);

        // TODO: send events as they come
        // FIXME: Someone needs to review this code, I just turned on Copilot and let it write this, including this comment
        return Ok(Response::new(Box::pin(
            BroadcastStream::new(self.ev_stream.subscribe()).filter_map(|x| x.ok()),
        )));
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
