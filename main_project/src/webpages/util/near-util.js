import Big from 'big.js'

export const NETWORK_ID = process.env.REACT_APP_NETWORK_ID || 'default'

export const ACCESS_KEY_ALLOWANCE = Big(1000000000).times(10 ** 24).toFixed()
export const MAX_UINT8 = '340282366920938463463374607431768211455'
export const BOATLOAD_OF_GAS = Big(2).times(10 ** 14).toFixed()
export const DROP_GAS = Big(75).times(10 ** 12).toFixed()
export const MAX_TGAS = Big(30).times(10 ** 12).toFixed()
export const MULTISENDER_GAS = Big(300).times(10 ** 12).toFixed()
const APPROX_ZERO_MIN = 10

export const toYocto = (value = '0') => Big(value).times(10 ** 24).toFixed()
export const yoctoTo = (value = '0', to = 2) => Big(value).div(10 ** 24).toFixed(to === 0 ? undefined : to)
export const big = (value = '0') => Big(value)
export const gtZero = (value = '0') => big(value).gt(big())
export const gtZeroApprox = (value = '0') => big(value).gt(big(APPROX_ZERO_MIN))