import React from "react";

const logout_render = (login) => {
  return (
    <main>
      <h1>Welcome to NEAR!</h1>
      <p>
        Render main works!
      </p>
      <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
        <button onClick={login}>Sign in</button>
      </p>
    </main>
  )
}

export { logout_render };