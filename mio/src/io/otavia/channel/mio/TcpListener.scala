package io.otavia.channel.mio

import io.github.otavia.jni.loader.NativeLoader
import sun.nio.cs.StandardCharsets

import java.nio.charset.Charset

class TcpListener private(val raw: Long) extends NativeLoader("mionative") with MioSocket {

  override val socketId: Int = MioSocket.getNextId

  def accept(): TcpStream = new TcpStream(accept0(raw))

  @native private def accept0(raw: Long): Long
  

}

object TcpListener extends NativeLoader("mionative") {
  def bind(addr: String): TcpListener = new TcpListener(bind0(addr))


  @native def bind0(addr: String): Long
}
