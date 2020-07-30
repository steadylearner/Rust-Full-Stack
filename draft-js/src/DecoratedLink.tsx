import * as React from "react";
import { EditorContext } from "./EditorContext";

export function DecoratedLink({
    className,
    children,
    entityKey,
    target
}: {
    className: string;
    children: React.ReactNode;
    entityKey: string;
    target?: string;
}) {
    const editorState = React.useContext(EditorContext);
    const entity = editorState.getCurrentContent().getEntity(entityKey);
    const entityData = entity ? entity.getData() : undefined;
    const href = (entityData && entityData.url) || undefined;

    return (
        <a
            className={className}
            title={href}
            href={href}
            target={target}
            rel="noopener"
        >
            {children}
        </a>
    );
}