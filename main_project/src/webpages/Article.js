import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import * as nearApi from 'near-api-js'
import {
  MAX_TGAS
} from './util/near-util'


const add_new_article_default1 = (article_number) => {
  async function add_new_article_default1() {
    try {
      await contract.add_new_article_default({ article_number }, MAX_TGAS) 
    } catch(e) {
      console.warn(e)
    }
  }

  return (
    <button onClick={() => add_new_article_default1()}>Save and Create</button>
  )
}


export default add_new_article_default1;