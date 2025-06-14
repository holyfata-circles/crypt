import test from 'ava'

import { Crypt } from '../index.js'

test('base64ToBytes simple', (t) => {
  // "TWFu" -> "Man"
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("TWFu")), Buffer.from("Man"))
})

test('base64ToBytes with padding', (t) => {
  // "TWE=" -> "Ma"
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("TWE=")), Buffer.from("Ma"))
  // "TQ==" -> "M"
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("TQ==")), Buffer.from("M"))
})

test('base64ToBytes with invalid chars', (t) => {
  // "T W F u" -> "Man"
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("T W F u")), Buffer.from("Man"))
})

test('base64ToBytes empty', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("")), Buffer.alloc(0))
})

test('base64ToBytes all ascii', (t) => {
  // "QUJDREVGR0hJSg==" -> "ABCDEFGHIJ"
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("QUJDREVGR0hJSg==")), Buffer.from("ABCDEFGHIJ"))
})

test('base64ToBytes non ascii', (t) => {
  // "5Lit5paH" -> "中文"
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("5Lit5paH")), Buffer.from("中文"))
})

test('base64ToBytes with symbols', (t) => {
  // "ISIjJCVeJiooKQ==" -> "!\"#$%^&*()"
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("ISIjJCVeJiooKQ==")), Buffer.from("!\"#$%^&*()"))
})
