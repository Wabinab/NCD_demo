import React from "react";
import "../global.css"
import "./local.css"

const user_page_render = (login) => {
  return (
    <main>
        <div class="topnav">
          <div class="topnav-right">
            <button onClick={login}>Sign in</button>
          </div>
          <img src={require('/assets/near.png')} width="2%" height="2%"/>
          <h1>Some Title</h1>
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