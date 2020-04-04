const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:8000/ws');

ws.on('message', function incoming(data) {
  console.log(data);
});
ws.on('open', function open() {
ws.send('i_wanna_listen');
});


//const ws = new WebSocket('ws://localhost:8000/ws');
//
//ws.on('open', function open() {
//  ws.send('something');
//});
//



//const ws2 = new WebSocket('ws://localhost:8000/ws');
//
//ws2.on('open', function open() {
//  ws2.send('something');
//});
//
//ws2.on('message', function incoming(data) {
//  console.log(listener);
//  console.log(data);
//});
