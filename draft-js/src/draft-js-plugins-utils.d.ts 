declare module "draft-js-plugins-utils" {
  import { EditorState, DraftEntityType, EntityInstance } from "draft-js";

  function createLinkAtSelection(
    editorState: EditorState,
    url: string
  ): EditorState;

  function getCurrentEntityKey(editorState: EditorState): string | null;

  function getCurrentEntity(editorState: EditorState): EntityInstance;

  function hasEntity(
    editorState: EditorState,
    entityType: DraftEntityType
  ): boolean;

  //@ts-ignore
  declare const _default = {
    createLinkAtSelection,
    getCurrentEntityKey,
    getCurrentEntity,
    hasEntity
  };

  export default _default;
}