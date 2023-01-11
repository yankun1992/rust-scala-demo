package io.otavia.channel.mio

enum Interest(val value: Int) {
  case READABLE extends Interest(1)
  case WRITABLE extends Interest(1 << 1)
}