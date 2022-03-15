import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import * as nearApi from 'near-api-js'
import * as clipboard from "clipboard-polyfill/text";
import {
  nearTo, nearToInt, toNear, BOATLOAD_OF_GAS, DROP_GAS, NETWORK_ID, ACCESS_KEY_ALLOWANCE
} from './util/near-util'

import { useNavigate } from "react-router-dom";

const get = (k) => JSON.parse(localStorage.getItem(k) || '[]')
const set = (k, v) => localStorage.setItem(k, JSON.stringify(v))


const Drops = () => {
    const {
      contractName, 
      walletUrl
    } = window.nearConfig

    // const {
    //   currentUser, currentUser: { account_id }, updateUser
    // } = props

    // let accountId = account_id
    // if (!accountId || accountId.length === 0) {
    //   accountId = window.prompt('Your AccountId?')
    // }

    const dropStorageKey = '__drops_' + accountId

    const [drops, set_drops] = useState([])
    const [showUsed, set_show_used] = useState(false)
    const [urlDrop, set_url_drop] = useState()

    useEffect(() => {
      update_drops(true)
      const url = new URL(window.location.href)
      const key = url.searchParams.get('key')
      const amount = url.searchParams.get('amount')
      const from = url.searchParams.get('from')
      const limited = url.searchParams.get('limited') === 'true'

      if (key && amount && from) {
        set_url_drop({ key, amount, from, limited })
      }
    }, [])


    /*****************
     * Update Drops
     *****************/

    async function get_drop(public_key) {
      const drops = await get(dropStorageKey) || []
      return drops.find((d) => d.public_key === public_key)
    }


    async function update_drops(check = false) {
      const drops = (await get(dropStorageKey) || [])

      for (let drop of drops) {
        const { public_key: key } = drop
        drop.walletLink = await get_wallet_link(key)

        if (!check) {
          continue
        }

        const { contract } = window
        let res
        try {
          res = await contract.get_key_balance({ key })
        } catch (e) {
          console.warn(e)

          if (e.message.indexOf('Key is missing') > -1) {
            await use_drop(key)
          }
        }
      }

      set_drops(drops)
    }

    async function use_drop(public_key) {
      const drops = await get(dropStorageKey) || []
      drops.splice(drops.findIndex((d) => d.public_key === public_key), 1)
      await set (dropStorageKey, drops)
      update_drops()
    }

    async function add_drop(newKeyPair) {
      const drops = await get(dropStorageKey) || []
      drops.push(newKeyPair)
      await set(dropStorageKey, drops)
      update_drops()
    }

    async function get_wallet_link(public_key) {
      const { secretKey } = await get_drop(public_key)
      return `${walletUrl}/create/${contractName}/${secretKey}`
    }

    function download_file(fileName, data, type='text/plain') {
      const a = document.createElement('a')
      a.style.display = 'none'
      document.body.appendChild(a)
      a.href = window.URL.createObjectURL(new Blob([data], { type }))
      a.setAttribute("download", fileName)
      a.click()
      window.URL.revokeObjectURL(a.href)
      document.body.removeChild(a)
    }

    async function get_contract(viewMethods = [], changeMethods = [], secretKey) {
      if (secretKey) {
        await window.keyStore.setKey(
          NETWORK_ID, contractName,
          nearApi.KeyPair.fromString(secretKey)
        )
      }

      const contract = new nearApi.Contract(window.contractAccount, contractName, {
        viewMethods,
        changeMethods,
        sender: contractName
      })

      return contract
    }

    /********************
     * Fund the Drop
     * CAREFUL of not saving user's secret key. 
     ********************/
    async function send_drop() {
      // 0.1 CREATE ACCOUNT + 0.01 ACCESS_KEY_ALLOWANCE
      const amount = toNear('0.11' || 0)
      
      const newKeyPair = nearApi.KeyPair.fromRandom('ed25519')
      const public_key = newKeyPair.public_key = newKeyPair.publicKey.toString().replace('ed25519:', '')

      // const downloadKey = window.confirm("PLEASE DO NOT LOST YOUR KEY. \
      //   Backup, Backup, Backup! We don't store \
      //   your key, so IF YOU LOSE IT, YOU LOSE YOUR MONEY FOREVER! We can never \
      //   recover your funds for you. YOUR MONEY, YOUR RESPONSIBILITY. \
      //   We'll download your key after this is close, no matter you confirm or not. ")
      
        const { secretKey, public_key: publicKey } = JSON.parse(JSON.stringify(newKeyPair))
        download_file(public_key + '.txt', JSON.stringify({ publicKey, secretKey }))

        newKeyPair.amount = amount
        newKeyPair.ts = Date.now()
        await add_drop(newKeyPair)

        // const { contract } = window
        // try {
        //   await contract.send({ public_key }, DROP_GAS) 
        // } catch(e) {
        //   console.warn(e)
        // }

        return get_wallet_link(public_key);
    }

    const activeDrops = drops.filter((d) => !d.used).sort((a, b) => b.ts - a.ts)
    const usedDrops = drops.filter((d) => d.used).sort((a, b) => b.ts - a.ts)

    const mapped_activeDrops = new Map(
      activeDrops.map(
        ({ public_key, amount, ts, walletLink }) => 
        [public_key, walletLink]
      )
    );



    function signup() {

      // Remove previously used drops here. 
    
      send_drop().then((wallet_link) =>{ 
        window.open(wallet_link, "_blank");
      });  
    
    }

    return (
      <button onClick={() => signup()}>Sign up</button>
    )
}

export default Drops;