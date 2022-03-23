// import React from "react";
import "../global.css"
import "./local.css"
import NearLogo from "./assets/near-black.svg";
import Editor from "./Editor"
import add_new_article_default1 from "./Article";
import tip_author from "./Tip";



// =============================================

var article_map = {
  '/eat': 1,
  '/sleep': 2,
  '/survive': 3
};

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


const top_bar = (signup, login, username, signed_in) => {
  if (!signed_in) {
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
          <li><button onClick={login}>Sign out</button></li> 
        </ul>
      </div>
  )
}


const render_based_on_page = (current_page, signup, login, username, signed_in) => {
  if (!signed_in) {
    return (
      <main>
        {top_bar(signup, login, username, signed_in)}
        <p>In {current_page} path</p> 
        <Editor />
        <a href="/">Home</a>
      </main>
    )
  }
  
  return (
    <main>
      {top_bar(signup, login, username, signed_in)}
      <p>In {current_page} path</p> 
      <Editor />
      <a href="/">Home</a>
      {add_new_article_default1(article_map[current_page])}
      <button>Edit and Save</button>
      <div>
        {tip_author()}
      </div>
    </main>
  )
}


const user_page_render = (signup, login, username, articles, current_page, signed_in) => {
  if (current_page == "/") {
    return (
      <main>
        {top_bar(signup, login, username, signed_in)}

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
    return render_based_on_page(current_page, signup, login, username, signed_in)
  }
}

export { user_page_render };