import React from "react";
import "../global.css"
import "./local.css"

const user_page_render = (login) => {
  return (
    <main>
      <div class="titlediv">
        {/* <div class="logo">
          <img src="../assets/logo-black.svg" alt="" />
        </div> */}
        
        <div class="topnav">
          <h1>Some Title</h1>
          <div class="topnav-right">
            <button onClick={login}>Sign in</button>
          </div>
        </div>
      </div>

      <div>
        <p>
          Main paragraph
        </p>
      </div>
    </main>
  )
}

export { user_page_render };