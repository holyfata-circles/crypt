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

test('hexToBytes basic', (t) => {
  t.deepEqual(Buffer.from(Crypt.hexToBytes("4d616e")), Buffer.from("Man"))
  t.deepEqual(Buffer.from(Crypt.hexToBytes("48656c6c6f")), Buffer.from("Hello"))
})

test('hexToBytes odd length', (t) => {
  // 忽略最后一个不完整的字符
  t.deepEqual(Buffer.from(Crypt.hexToBytes("4d61f")), Buffer.from([0x4d, 0x61]))
})

test('hexToBytes invalid', (t) => {
  // 非法字符被忽略
  t.deepEqual(Buffer.from(Crypt.hexToBytes("zz4d61")), Buffer.from([0x4d, 0x61]))
})

test('hexToBytes empty', (t) => {
  t.deepEqual(Buffer.from(Crypt.hexToBytes("")), Buffer.alloc(0))
})

test('bytesToHex basic', (t) => {
  t.is(Crypt.bytesToHex(Buffer.from("Man")), "4d616e")
  t.is(Crypt.bytesToHex(Buffer.from("Hello")), "48656c6c6f")
})

test('bytesToHex empty', (t) => {
  t.is(Crypt.bytesToHex(Buffer.alloc(0)), "")
})

test('bytesToHex symbols', (t) => {
  t.is(Crypt.bytesToHex(Buffer.from("!\"#$%")), "2122232425")
})

test('bytesToWords basic', (t) => {
  // [0x4d, 0x61, 0x6e, 0x48] -> [0x4d616e48]
  t.deepEqual(Crypt.bytesToWords(Buffer.from([0x4d, 0x61, 0x6e, 0x48])), [0x4d616e48])
})

test('bytesToWords multiple words', (t) => {
  // [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x22, 0x23] -> [0x48656c6c, 0x6f212223]
  t.deepEqual(
    Crypt.bytesToWords(Buffer.from([0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x22, 0x23])),
    [0x48656c6c, 0x6f212223]
  )
})

test('bytesToWords empty', (t) => {
  t.deepEqual(Crypt.bytesToWords(Buffer.alloc(0)), [])
})

test('bytesToWords not multiple of 4', (t) => {
  // [0x12, 0x34, 0x56] -> [0x12345600]
  t.deepEqual(Crypt.bytesToWords(Buffer.from([0x12, 0x34, 0x56])), [0x12345600])
})

test('wordsToBytes basic', (t) => {
  // 0x4d616e48 = [0x4d, 0x61, 0x6e, 0x48]
  t.deepEqual(
    Buffer.from(Crypt.wordsToBytes([0x4d616e48])),
    Buffer.from([0x4d, 0x61, 0x6e, 0x48])
  )
})

test('wordsToBytes multiple words', (t) => {
  // [0x48656c6c, 0x6f212223] -> [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x22, 0x23]
  t.deepEqual(
    Buffer.from(Crypt.wordsToBytes([0x48656c6c, 0x6f212223])),
    Buffer.from([0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x22, 0x23])
  )
})

test('wordsToBytes empty', (t) => {
  t.deepEqual(Buffer.from(Crypt.wordsToBytes([])), Buffer.alloc(0))
})

test('randomBytes length', (t) => {
  const bytes = Crypt.randomBytes(16)
  t.is(bytes.length, 16)
})

test('randomBytes randomness', (t) => {
  const a = Crypt.randomBytes(8)
  const b = Crypt.randomBytes(8)
  // 极小概率相等，这里只做基本不等性测试
  t.not(Buffer.from(a).toString('hex'), Buffer.from(b).toString('hex'))
})

test('randomBytes zero', (t) => {
  const bytes = Crypt.randomBytes(0)
  t.is(bytes.length, 0)
})

test('rotl basic', (t) => {
  t.is(Crypt.rotl(0x12345678, 8), 0x34567812)
  t.is(Crypt.rotl(0x12345678, 0), 0x12345678)
  t.is(Crypt.rotl(0x12345678, 32), 0x12345678)
})

test('rotr basic', (t) => {
  t.is(Crypt.rotr(0x12345678, 8), 0x78123456)
  t.is(Crypt.rotr(0x12345678, 0), 0x12345678)
  t.is(Crypt.rotr(0x12345678, 32), 0x12345678)
})
