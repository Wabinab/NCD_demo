import React from "react";
import "../global.css"
import "./local.css"
import NearLogo from "./assets/near-black.svg";

const user_page_render = (login) => {
  return (
    <main>
      <div class="topnav">
        <div class="logo">
          <img src={NearLogo} alt="NEAR Logo" height="30%"/>
          <h1> NEAR Blog</h1>
        </div>
        <ul class="middle">
          <li><h2>User Profile</h2></li>
        </ul>
        <ul class="no-bullets">
          <li><button onClick={login}>Sign in</button></li> 
        </ul>
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