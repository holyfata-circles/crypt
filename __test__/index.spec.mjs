import test from 'ava'

import { Crypt } from '../index.js'

test('base64ToBytes simple', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("TWFu")), Buffer.from("Man"))
})

test('base64ToBytes with padding', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("TWE=")), Buffer.from("Ma"))
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("TQ==")), Buffer.from("M"))
})

test('base64ToBytes with invalid chars', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("T W F u")), Buffer.from("Man"))
})

test('base64ToBytes empty', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("")), Buffer.alloc(0))
})

test('base64ToBytes all ascii', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("QUJDREVGR0hJSg==")), Buffer.from("ABCDEFGHIJ"))
})

test('base64ToBytes non ascii', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("5Lit5paH")), Buffer.from("中文"))
})

test('base64ToBytes with symbols', (t) => {
  t.deepEqual(Buffer.from(Crypt.base64ToBytes("ISIjJCVeJiooKQ==")), Buffer.from("!\"#$%^&*()"))
})

test('bytesToBase64 simple', (t) => {
  t.is(Crypt.bytesToBase64(Buffer.from("Man")), "TWFu")
})

test('bytesToBase64 with padding', (t) => {
  t.is(Crypt.bytesToBase64(Buffer.from("Ma")), "TWE=")
  t.is(Crypt.bytesToBase64(Buffer.from("M")), "TQ==")
})

test('bytesToBase64 all ascii', (t) => {
  t.is(Crypt.bytesToBase64(Buffer.from("ABCDEFGHIJ")), "QUJDREVGR0hJSg==")
})

test('bytesToBase64 non ascii', (t) => {
  t.is(Crypt.bytesToBase64(Buffer.from("中文")), "5Lit5paH")
})

test('bytesToBase64 with symbols', (t) => {
  t.is(Crypt.bytesToBase64(Buffer.from("!\"#$%^&*()")), "ISIjJCVeJiooKQ==")
})

test('bytesToBase64 empty', (t) => {
  t.is(Crypt.bytesToBase64(Buffer.alloc(0)), "")
})
