import { beforeAll, describe, it, expect } from 'vitest'

describe('@verre/core', () => {
  // @ts-expect-error missing types
  beforeAll(async () => import('../bench/verre.js'))

  it('example', async () => {
    const res1 = await fetch('http://127.0.0.1:3000/')
    const res1Text = await res1.text()
    expect(res1Text).toBe('Hi')

    const res2 = await fetch('http://127.0.0.1:3000/id/1?name=bun')
    expect(res2.headers.get('x-powered-by')).toBe('benchmark')

    const res2Text = await res2.text()
    expect(res2Text).toBe('1 bun')

    const res3Body = JSON.stringify({ hello: 'world' })
    const res3 = await fetch('http://127.0.0.1:3000/json', {
      method: 'POST',
      body: res3Body,
      headers: { 'content-type': 'application/json', 'content-length': res3Body.length.toString() },
    })
    expect(res3.headers.get('content-type')).toBe('application/json')

    const res3Text = await res3.text()
    expect(res3Text).toBe(res3Body)
  })
})
