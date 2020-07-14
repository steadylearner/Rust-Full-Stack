import * as React from "react";
import { EditorState } from "draft-js";

export const EditorContext = React.createContext<EditorState>(
    EditorState.createEmpty()
);