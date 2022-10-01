const $status = document.querySelector('#status')
const $connectButton = document.querySelector('#connect')
const $log = document.querySelector('#log')
const $form = document.querySelector('#chatform')
const $input = document.querySelector('#text')

var socket = null

function log(msg, type = 'status') {
    $log.innerHTML += `<p class="msg msg--${type}">${msg}</p>`
    $log.scrollTop += 1000
}

function connect() {
    disconnect()

    const { location } = window

    const proto = location.protocol.startsWith('https') ? 'wss' : 'ws'
    const wsUri = `${proto}://${location.host}/ws`

    log('Connecting...')
    socket = new WebSocket(wsUri)

    socket.onopen = () => {
      log('Connected')
      updateConnectionStatus()
    }

    socket.onmessage = (ev) => {
      log('Received: ' + ev.data, 'message')
    }

    socket.onclose = () => {
      log('Disconnected')
      socket = null
      updateConnectionStatus()
    }
}

function disconnect() {
  if (socket) {
    log('Disconnecting...')
    socket.close()
    socket = null
    updateConnectionStatus()
  }
}

function updateConnectionStatus() {
        if (socket) {
          $status.style.backgroundColor = 'transparent'
          $status.style.color = 'green'
          $status.textContent = `connected`
          $connectButton.innerHTML = 'Disconnect'
          $input.focus()
        } else {
          $status.style.backgroundColor = 'red'
          $status.style.color = 'white'
          $status.textContent = 'disconnected'
          $connectButton.textContent = 'Connect'
        }
}

$connectButton.addEventListener('click', () => {
        if (socket) {
          disconnect()
        } else {
          connect()
        }

        updateConnectionStatus()
})

$form.addEventListener('submit', (ev) => {
  ev.preventDefault()

  const text = $input.value

  log('Sending: ' + text)
  socket.send(text)

  $input.value = ''
  $input.focus()
})

updateConnectionStatus()
