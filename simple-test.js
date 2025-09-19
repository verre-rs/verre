import { Verre, Response } from './index.js'

const app = new Verre()

app.get('/', () => Response.text('Hello from Verre!'))

app.serve()
