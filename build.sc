import mill._
import scalalib._
import mill.api.Loose
import $ivy.`io.github.otavia-projects::mill-rust_mill$MILL_BIN_PLATFORM:0.1.0`
import io.github.otavia.jni.plugin.RustJniModule

object mio extends ScalaModule {
  override def scalaVersion = "3.2.1"

  override def ivyDeps: T[Loose.Agg[Dep]] = Agg(
    ivy"io.netty:netty5-buffer:5.0.0.Alpha5",
    ivy"io.github.otavia-projects::jni-loader::0.1.0"
  )

  override def moduleDeps: scala.Seq[JavaModule] = scala.Seq(mionative)

}

object mionative extends RustJniModule {

  override def release: Boolean = false

}




