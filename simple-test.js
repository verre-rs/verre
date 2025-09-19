import { Verre, Response } from './index.js'

const app = new Verre()

app.get('/', () => Response.new('Hello from Verre!'))

app.serve()
