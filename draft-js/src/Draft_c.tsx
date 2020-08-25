// Include metions while you refer to this.(draft-js-mention-plugin")
// https://codesandbox.io/s/8n3zl4n9pl?file=/src/mentions.js
// https://github.com/draft-js-plugins/draft-js-aplugins/tree/master/draft-js-mention-plugin
// https://www.draft-js-plugins.com/plugin/mention
// https://stackoverflow.com/questions/60590880/draft-js-mention-plugin-change-mentions-object-k-v
// https://www.google.com/search?client=firefox-b-d&q=draft+js+mention+plguin+use+other+values

// Should refer to this part.
// https://github.com/draft-js-plugins/draft-js-plugins/blob/master/draft-js-mention-plugin/src/MentionSuggestions/index.js#L236

// You can see that name is used for substitued value.
// https://github.coam/draft-js-plugins/draft-js-plugins/bloba/mastera/draft-js-mention-plugin/src/modifiers/addMention.js

// Make user suggestion # instead of @ following the documenations above.

// Send the project.

import * as React from "react";
import { EditorContext } from "./EditorContext";

import {
    Box,
    Paper,
    TextField,
    InputAdornment,
    IconButton,
    Typography,
} from "@material-ui/core";
import { ToggleButtonGroup, ToggleButton } from "@material-ui/lab";
import { 
    AddAPhoto, 
    Link, 
    Check 
} from "@material-ui/icons";

import {
    Editor,
    EditorState,
    Modifier,
    CompositeDecorator,
    AtomicBlockUtils,
    getVisibleSelectionRect,
    convertToRaw, // JSON
    ContentBlock
} from "draft-js";
import draftUtils from "draft-js-plugins-utils";
import createMentionPlugin, {
  defaultSuggestionsFilter
} from "draft-js-mention-plugin";
import "draft-js-mention-plugin/lib/plugin.css";

import { Mentions, mentions } from "./mentions";

import { linkStrategy } from "./linkStrategy";
import { DecoratedLink } from "./DecoratedLink";
import { MediaComponent } from "./MediaComponent";


// interface IProps {
// 	variables: Array<{id: string, name: string}>
// 	state: EditorState
// 	onChange(state: EditorState): void
// }

// <Editor {...props } />

// Should modify <Editor></Editor> part below with them.

export default function Draft({
    user,
    author,
}) {
    // 1. Focus
    const editor = React.useRef<Editor>(null);

    const focusEditor = React.useCallback(() => {
        if (editor.current) {
            editor.current.focus();
        }
    }, [editor]);

    // 2. Menu(toggle bars)
    const [toggleButtonGroupValue, setToggleButtonGroupValue] = React.useState<
        string | null
    >(null);

    const handleToggleButtonGroup = (
        event: React.MouseEvent<HTMLElement, MouseEvent>,
        value: string
    ) => {
        event.stopPropagation();
        event.preventDefault();
        setToggleButtonGroupValue(value);

        switch (value) {
            case "header-one":
                setEditorState(
                    EditorState.push(
                        editorState,
                        Modifier.setBlockType(
                            editorState.getCurrentContent(),
                            editorState.getSelection(),
                            value
                        ),
                        "change-block-type"
                    )
                );
                break;
            case "header-two":
                setEditorState(
                    EditorState.push(
                        editorState,
                        Modifier.setBlockType(
                            editorState.getCurrentContent(),
                            editorState.getSelection(),
                            value
                        ),
                        "change-block-type"
                    )
                );
                break;
            case "BOLD":
                setEditorState(
                    EditorState.push(
                        editorState,
                        Modifier.applyInlineStyle(
                            editorState.getCurrentContent(),
                            editorState.getSelection(),
                            value
                        ),
                        "change-inline-style"
                    )
                );
                break;
            case "ITALIC":
                setEditorState(
                    EditorState.push(
                        editorState,
                        Modifier.applyInlineStyle(
                            editorState.getCurrentContent(),
                            editorState.getSelection(),
                            value
                        ),
                        "change-inline-style"
                    )
                );
                break;
            case "insert-image":
                //@ts-ignore
                document.getElementById("fileInput").click();
                setToggleButtonGroupValue("null"); // What is diffrent with null?
                break;
            case "insert-link":
                break;
            default:
                setToggleButtonGroupValue(null); // // What is diffrent with "null"?
                break;
        }
    };

    // 3. Text editor
    const [editorState, setEditorState] = React.useState<EditorState>(
        EditorState.createEmpty(
            new CompositeDecorator([
                {
                    strategy: linkStrategy,
                    component: DecoratedLink
                }
            ])
        ),
    );

    // 4. Rectangle menu
    const [selectionRect, setSelectionRect] = React.useState<{
        left: number;
        width: number;
        right: number;
        top: number;
        bottom: number;
        height: number;
    }>({ left: 0, width: 0, right: 0, top: 0, bottom: 0, height: 0 });

    const renderBlock = (contentBlock: ContentBlock) => {
        if (contentBlock.getType() === "atomic") {
            const entityKey = contentBlock.getEntityAt(0);
            const entityData = editorState
                .getCurrentContent()
                .getEntity(entityKey)
                .getData();
            return {
                component: MediaComponent,
                editable: false,
                props: {
                    src: { file: entityData.src }
                }
            };
        }
    };

    React.useEffect(() => {
        if (getVisibleSelectionRect(window) !== null) {
            setSelectionRect(getVisibleSelectionRect(window));
        }
    }, [editorState, setSelectionRect]);

    // 5. anchorElUrl is this directly relevant to 4.?

    const [anchorElUrl, setAnchorElUrl] = React.useState<string>("");

    // 6.

    const editorPlaceholder = `Hey ${user.name}, hope you are doing well. You can reach to us ${author.email}.`

    // 7. Mention plugin

    const mentionPlugin = createMentionPlugin({
        mentions,
        entityMutability: 'IMMUTABLE',
        mentionPrefix: '#',
    });

    const [suggestions, setSugesstions] = React.useState<Mentions>(mentions);

    const onSearchChange = ({ value }) => {
        const newSugesstion = defaultSuggestionsFilter(value, mentions);
        setSugesstions(newSugesstion);
    };

    const onAddMention = (mention) => {
        console.log(mention);
    };

    const { MentionSuggestions } = mentionPlugin;
    const plugins = [mentionPlugin];

    return (
        <Box>
            <Box m={2}>
                <Box mb={2}>
                    <Typography variant="h2" component="h2" gutterBottom>
                        Test draft-js with TypeScript
                    </Typography>
                    <ToggleButtonGroup
                        exclusive
                        onChange={handleToggleButtonGroup}
                        value={toggleButtonGroupValue}
                    >
                        <ToggleButton value="header-one">
                            H1
                        </ToggleButton>
                        <ToggleButton value="header-two">
                            H2
                        </ToggleButton>
                        <ToggleButton value="BOLD">
                            <strong>B</strong>
                        </ToggleButton>
                        <ToggleButton value="ITALIC">
                            <i>I</i>
                        </ToggleButton>
                        <ToggleButton value="insert-image">
                            <AddAPhoto />
                        </ToggleButton>
                        <ToggleButton
                            value="insert-link"
                            disabled={editorState.getSelection().isCollapsed()}
                        >
                            <Link />
                        </ToggleButton>
                    </ToggleButtonGroup>
                </Box>
                <Box>
                    <Paper style={{ minHeight: "144px" }}>
                        <Box onClick={focusEditor} p={4}>
                            <EditorContext.Provider value={editorState}>
                                <Editor
                                    editorState={editorState}
                                    onChange={setEditorState}
                                    placeholder={editorPlaceholder}
                                    blockRendererFn={renderBlock}
                                    plugins={plugins}
                                    ref={editor}
                                />
                            </EditorContext.Provider>
                            <MentionSuggestions
                                onSearchChange={onSearchChange}
                                suggestions={suggestions}
                                onAddMention={onAddMention}
                            />
                        </Box>
                    </Paper>
                </Box>
            </Box>
            <Box
                style={{
                    position: "absolute",
                    top: selectionRect.top,
                    left: selectionRect.right + 12,
                    background: "#FFF",
                    display: toggleButtonGroupValue === "insert-link" ? "block" : "none"
                }}
            >
                <TextField
                    variant="outlined"
                    InputProps={{
                        endAdornment: (
                            <InputAdornment position="start">
                                <IconButton
                                    onClick={() => {
                                        setEditorState(
                                            draftUtils.createLinkAtSelection(editorState, anchorElUrl)
                                        );
                                        setToggleButtonGroupValue(null);
                                    }}
                                >
                                    <Check />
                                </IconButton>
                            </InputAdornment>
                        )
                    }}
                    placeholder="https://"
                    value={anchorElUrl}
                    onChange={e => setAnchorElUrl(e.target.value)}
                />
            </Box>
            <Box ml={3}>
                <pre>
                    {JSON.stringify(
                        convertToRaw(editorState.getCurrentContent()),
                        null,
                        2
                    )}
                </pre>
            </Box>
            <input
                id="fileInput"
                style={{ display: "none" }}
                type="file"
                accept="image/png,image/jpeg,image/jpg,image/gif"
                onChange={event => {
                    const reader = new FileReader();
                    reader.addEventListener(
                        "load",
                        function () {
                            const contentStateWithEntity = editorState
                                .getCurrentContent()
                                .createEntity("IMAGE", "IMMUTABLE", { src: reader.result });
                            setEditorState(
                                AtomicBlockUtils.insertAtomicBlock(
                                    EditorState.set(editorState, {
                                        currentContent: contentStateWithEntity
                                    }),
                                    contentStateWithEntity.getLastCreatedEntityKey(),
                                    " "
                                )
                            );
                        },
                        false
                    );
                    if (event.target.files) {
                        reader.readAsDataURL(
                            Array.prototype.slice.call(event.target.files)[0]
                        );
                    }
                }}
            />
        </Box>
    );
}
