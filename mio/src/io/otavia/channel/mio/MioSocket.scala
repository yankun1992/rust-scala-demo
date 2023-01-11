package io.otavia.channel.mio

import java.util.concurrent.atomic.AtomicInteger

trait MioSocket {
  def socketId: Int

  def raw: Long
}

object MioSocket {
  private val nextId = new AtomicInteger(0)

  def getNextId: Int = nextId.getAndIncrement()
}
