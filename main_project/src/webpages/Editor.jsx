import * as React from "react";

import "trix/dist/trix";
import "trix/dist/trix.css";

import { TrixEditor } from "react-trix";
import html from "./_temp_content";


const Editor = () => {
  const [content, setContent] = React.useState("");

  const handleEditorReady = editor => {
    editor.insertHTML(html);
  };
  const handleChange = (html, text) => {
    setContent(html);
    // some way to push it to backend and save, 
    // first locally, then delete and publish during 
    // globally. 
  };

  return (
    <>
      <TrixEditor
        onChange={handleChange}
        onEditorReady={handleEditorReady}
        value={content}
      />
      <div>
        <pre>{content}</pre>
      </div>
    </>
  );
};

export default Editor;
