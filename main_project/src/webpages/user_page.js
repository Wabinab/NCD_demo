// import React from "react";
import "../global.css"
import "./local.css"
import NearLogo from "./assets/near-black.svg";
import Editor from "./Editor"


// =============================================


const article_factory = article => {
  return(
    <li class="articles">
      <a href={article.target}>
        <h4 class="articles">{article.title}</h4>
        <p class="articles">{article.explanation}</p>
      </a>
    </li>
  )
}


const article_object = (articles) => {
  const article_array = articles.map((article) => 
    article_factory(article)
  );

  return (
    <div class="articles">
      <ul class="articles">
        {article_array}  
      </ul>
    </div>
  )
}


const top_bar = (signup, login, username) => {
  return (
    <div class="topnav">
        <div class="logo">
          <img src={NearLogo} alt="NEAR Logo" height="30%"/>
          <h1> near.blog</h1>
        </div>
        <ul class="middle">
          <li><h2>{username}</h2></li>
        </ul>
        <ul class="no-bullets">
          {/* <li><button onClick={() => signup()}>Sign up</button></li> */}
          <li>{signup()}</li>
          <li><button onClick={login}>Sign in</button></li> 
        </ul>
      </div>
  )
}


const render_based_on_page = (current_page, signup, login, username) => {
  return (
    <main>
      {top_bar(signup, login, username)}
      <p>In {current_page} path</p> 
      <Editor />
      <a href="/">Home</a>
    </main>
  )
}


const user_page_render = (signup, login, username, articles, current_page) => {
  if (current_page == "/") {
    return (
      <main>
        {top_bar(signup, login, username)}

        <div class="description">
          <ul class="description">
            <li class="narrow"><p>
              A software developer, a machine learning engineer, 
              and loves to learn! Started to learn blockchain at 
              end of December 2021, and smart contract programming
              starting January 2022. 
            </p></li>
            <li class="narrow"><p>
              (a href for personal branding goes here.)
            </p></li>
          </ul>
        </div>

        {/* List of articles */}
        {article_object(articles)}
      </main>
    )
  } else {
    return render_based_on_page(current_page, signup, login, username)
  }
}

export { user_page_render };