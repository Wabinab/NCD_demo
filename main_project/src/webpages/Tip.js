import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import * as nearApi from 'near-api-js'
import {
  MULTISENDER_GAS, toYocto
} from './util/near-util'


async function tipping(article_id, tip_amount) {
  try {
    await contract.send_payout({ article_id }, MULTISENDER_GAS, tip_amount) 
  } catch(e) {
    console.warn(e)
  }
}


export default function tip_author() {
    const [inputs, setInputs] = useState({});

    const handleChange = (event) => {
      const name = event.target.name;
      const value = event.target.value;
      setInputs(values => ({...values, [name]: value}))
    }

    const handleSubmit = (event) => {
      event.preventDefault();
      tipping(inputs.article_id, toYocto(inputs.tip_amount));
      alert(`You tipped ${inputs.article_id} to ${inputs.tip_amount}`);
    }

    return (
      <form onSubmit={handleSubmit}>
        <label>Article Id:
          <input
            type="text"
            name="article_id"
            value={inputs.article_id || ""}
            onChange={handleChange}
          />
        </label>
        <label>Tip Amount:
          <input
            type="number"
            step="0.001"
            name="tip_amount"
            value={inputs.tip_amount || ""}
            onChange={handleChange}
          />
        </label>
        <input type="submit" />
      </form>
    )
}