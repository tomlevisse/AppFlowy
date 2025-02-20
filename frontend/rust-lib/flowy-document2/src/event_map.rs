use std::sync::Weak;

use strum_macros::Display;

use flowy_derive::{Flowy_Event, ProtoBuf_Enum};
use lib_dispatch::prelude::AFPlugin;

use crate::event_handler::convert_document;
use crate::event_handler::get_snapshot_handler;
use crate::{event_handler::*, manager::DocumentManager};

pub fn init(document_manager: Weak<DocumentManager>) -> AFPlugin {
  AFPlugin::new()
    .name(env!("CARGO_PKG_NAME"))
    .state(document_manager)
    .event(DocumentEvent::CreateDocument, create_document_handler)
    .event(DocumentEvent::OpenDocument, open_document_handler)
    .event(DocumentEvent::CloseDocument, close_document_handler)
    .event(DocumentEvent::ApplyAction, apply_action_handler)
    .event(DocumentEvent::GetDocumentData, get_document_data_handler)
    .event(
      DocumentEvent::ConvertDataToDocument,
      convert_data_to_document,
    )
    .event(DocumentEvent::Redo, redo_handler)
    .event(DocumentEvent::Undo, undo_handler)
    .event(DocumentEvent::CanUndoRedo, can_undo_redo_handler)
    .event(DocumentEvent::GetDocumentSnapshots, get_snapshot_handler)
    .event(DocumentEvent::CreateText, create_text_handler)
    .event(DocumentEvent::ApplyTextDeltaEvent, apply_text_delta_handler)
    .event(DocumentEvent::ConvertDocument, convert_document)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "FlowyError"]
pub enum DocumentEvent {
  #[event(input = "CreateDocumentPayloadPB")]
  CreateDocument = 0,

  #[event(input = "OpenDocumentPayloadPB", output = "DocumentDataPB")]
  OpenDocument = 1,

  #[event(input = "CloseDocumentPayloadPB")]
  CloseDocument = 2,

  #[event(input = "ApplyActionPayloadPB")]
  ApplyAction = 3,

  #[event(input = "OpenDocumentPayloadPB", output = "DocumentDataPB")]
  GetDocumentData = 4,

  #[event(input = "ConvertDataPayloadPB", output = "DocumentDataPB")]
  ConvertDataToDocument = 5,

  #[event(
    input = "DocumentRedoUndoPayloadPB",
    output = "DocumentRedoUndoResponsePB"
  )]
  Redo = 6,

  #[event(
    input = "DocumentRedoUndoPayloadPB",
    output = "DocumentRedoUndoResponsePB"
  )]
  Undo = 7,

  #[event(
    input = "DocumentRedoUndoPayloadPB",
    output = "DocumentRedoUndoResponsePB"
  )]
  CanUndoRedo = 8,

  #[event(input = "OpenDocumentPayloadPB", output = "RepeatedDocumentSnapshotPB")]
  GetDocumentSnapshots = 9,

  #[event(input = "TextDeltaPayloadPB")]
  CreateText = 10,

  #[event(input = "TextDeltaPayloadPB")]
  ApplyTextDeltaEvent = 11,

  /// Handler for converting a document to a JSON string, HTML string, or plain text string.
  ///
  /// ConvertDocumentPayloadPB is the input of this event.
  /// ConvertDocumentResponsePB is the output of this event.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```txt
  /// // document: [{ "block_id": "1", "type": "paragraph", "data": {"delta": [{ "insert": "Hello World!" }] } }, { "block_id": "2", "type": "paragraph", "data": {"delta": [{ "insert": "Hello World!" }] }
  /// let test = DocumentEventTest::new().await;
  /// let view = test.create_document().await;
  /// let payload = ConvertDocumentPayloadPB {
  ///   document_id: view.id,
  ///   range: Some(RangePB {
  ///     start: SelectionPB {
  ///       block_id: "1".to_string(),
  ///       index: 0,
  ///       length: 5,
  ///     },
  ///     end: SelectionPB {
  ///       block_id: "2".to_string(),
  ///       index: 5,
  ///       length: 7,
  ///     }
  ///   }),
  ///   export_types: ConvertTypePB {
  ///     json: true,
  ///     text: true,
  ///     html: true,
  ///   },
  /// };
  /// let result = test.convert_document(payload).await;
  /// assert_eq!(result.json, Some("[{ \"block_id\": \"1\", \"type\": \"paragraph\", \"data\": {\"delta\": [{ \"insert\": \"Hello\" }] } }, { \"block_id\": \"2\", \"type\": \"paragraph\", \"data\": {\"delta\": [{ \"insert\": \" World!\" }] } }".to_string()));
  /// assert_eq!(result.text, Some("Hello\n World!".to_string()));
  /// assert_eq!(result.html, Some("<p>Hello</p><p> World!</p>".to_string()));
  /// ```
  /// #
  #[event(
    input = "ConvertDocumentPayloadPB",
    output = "ConvertDocumentResponsePB"
  )]
  ConvertDocument = 12,
}
