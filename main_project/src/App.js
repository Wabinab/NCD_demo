import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'


import Drops from './webpages/Drops'

import { user_page_render } from "./webpages/user_page"
import { articles_db } from "./database/articles_db"

window.React = React;

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {
  // use React Hooks to store greeting in component state
  const [greeting, set_greeting] = React.useState()

  // when the user has not yet interacted with the form, disable the button
  const [buttonDisabled, setButtonDisabled] = React.useState(true)

  // after submitting the form, we want to show Notification
  const [showNotification, setShowNotification] = React.useState(false)

  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  React.useEffect(
    () => {
      // in this case, we only care to query the contract when signed in
      if (window.walletConnection.isSignedIn()) {

        // window.contract is set by initContract in index.js
        window.contract.get_greeting({ account_id: window.accountId })
          .then(greetingFromContract => {
            set_greeting(greetingFromContract)
          })
      }
    },

    // The second argument to useEffect tells React when to re-run the effect
    // Use an empty array to specify "only run on first render"
    // This works because signing into NEAR Wallet reloads the page
    []
  )

  

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return user_page_render(
      Drops,
      login, 
      "Wabinab", 
      articles_db, 
      window.location.pathname, 
      false,
    )
  }

  return user_page_render(
    Drops, 
    logout,
    "Wabinab",
    articles_db,
    window.location.pathname, 
    true, 
  )
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    // <>
      {/* <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button> */}
      
      {/* {showNotification && <Notification />} */}
    {/* </> */}
  
}


// this component gets rendered by App after the form is submitted
function Notification() {
  const urlPrefix = `https://explorer.${networkId}.near.org/accounts`
  return (
    <aside>
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.accountId}`}>
        {window.accountId}
      </a>
      {' '/* React trims whitespace around tags; insert literal space character when needed */}
      called method: 'set_greeting' in contract:
      {' '}
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.contract.contractId}`}>
        {window.contract.contractId}
      </a>
      <footer>
        <div>✔ Succeeded</div>
        <div>Just now</div>
      </footer>
    </aside>
  )
}
