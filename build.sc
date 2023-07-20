import mill._
import scalalib._
import mill.api.Loose
import $ivy.`io.github.otavia-projects::mill-rust_mill$MILL_BIN_PLATFORM:0.2.2`
import io.github.otavia.jni.plugin.RustJniModule

object mio extends ScalaModule {
  override def scalaVersion = "3.3.0"

  override def ivyDeps: T[Loose.Agg[Dep]] = Agg(
    ivy"io.netty:netty5-buffer:5.0.0.Alpha5",
    ivy"io.github.otavia-projects::jni-loader::0.2.2"
  )

  override def moduleDeps: scala.Seq[JavaModule] = scala.Seq(mionative)

}

object mionative extends RustJniModule {

  override def release: Boolean = false

}




