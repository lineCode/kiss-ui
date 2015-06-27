% Installing IUP

KISS-UI depends on the IUP library for C, created by Tecgraf, for a common abstraction on top of various operating systems (Windows, Linux) and their GUI systems (WinAPI, GTK+).

You will need to install the IUP binaries for your system, which are available for download [here][iup-dl].

Consult the following for which files to download and where to install them. The specific steps depend on your platform and preferred method of linking: dynamic or static.

PRs amending or adding instructions for any platform are very welcome.

[iup-dl]: http://sourceforge.net/projects/iup/files/3.14/

* [Windows](#windows)
* [Linux](#linux)
* [OS X](#os-x)

***
###Windows
####Dynamic linking
1. Navigate to `Windows Libraries/Dynamic`
  * 32-bit: Download `iup-3.14_Win32_dllw4_lib.zip`
  * 64-bit: Download `iup-3.14_Win64_dllw4_lib.zip`
2. Extract all `.dll` files to a folder where the linker can find them (pick one):
  * `<Rust install>/bin/rustlib/<platform target>/lib/` (recommended)
  * (using MinGW/MSYS) `<MinGW/MSYS install>/usr/lib`
  * `<Your cargo repository>/bin/<platform target>`
3. Copy the same DLLs to a folder in your PATH (pick one):
  * `<Rust install>/bin/` (recommended)
  * Create a folder anywhere and add it to your PATH.
  * Add one of the folders from step 2 to your PATH.

You should **NEVER** place arbitrary files in your Windows install folder, no matter how benign.

####Static Linking
Static linking with IUP on Windows is not currently possible as it requires resource scripts (`.rc`) files from IUP to be compiled and linked in, which Rust does not currently support.

***
###Linux
The Linux binary packages for IUP include both static and dynamic libraries. While efforts are underway to create up-to-date packages for various distributions' package managers, the currently most well supported methods of obtaining IUP binaries are to either compile them from source or download precompiled binaries from the creators.


#### Compile from Source
To compile from source, see [this page][iup-compile]. The instructions to check-out the source tree are available [here][iup-source]. If you understand how to build projects with Makefiles, then it shouldn't be too difficult.

#### Download the Precompiled Binaries
However, if you would rather download the precompiled binaries, begin by going to [the download page][iup-dl].

1. Navigate to the `Linux Libraries` folder.
2. Identify your kernel version. This can be done by entering the command `uname -r` into a terminal.
  * If you don't know if your Linux is 32-bit or 64-bit, use the command `uname -a` and look for the following:
    * `x86_64`: Your system is 64-bit.
    * `x86`: Your system is 32-bit.
3. Select and download the tarball for your kernel version and bit-arity.
  * For 32-bit (`x86`), there is only one package: `iup-3.14_Linux32_lib.tar.gz`
  * For 64-bit (`x86_64`), select one of the following based on your kernel version:
    * **>= 3.19**: `iup-3.14_Linux319_64_lib.tar.gz`
    * **>= 3.13**: `iup-3.14_Linux313_64_lib.tar.gz`
    * **>= 3.5**: `iup-3.14_Linux35_64_lib.tar.gz`
    * **>= 3.2**: `iup-3.14_Linux32_64_lib.tar.gz`
    * **2.6**: `iup-3.14_Linux26g4_64_lib.tar.gz`
4. Navigate to the folder where you downloaded the tarball to in a terminal.
5. Extract the tarball:
  * `mkdir iup_libs/`
  * `tar -xzvf <tarball file> -C iup_libs/`
6. Install the binaries:
  * `cd iup_libs/` (The install script must be run in its folder.)
  * You can run either, or both, of the following two commands:
  * To install the dynamic libraries: `sudo ./install`
  * To install the static libraries: `sudo ./install_dev`
7. Follow the prompts in the installer.

Once the installer completes, you are finished. If you later want to uninstall IUP, open that `iup_libs/` folder in a terminal and run `sudo ./uninstall`. Otherwise, you may now delete the tarball and/or the `iup_libs/` folder.

[iup-compile]: http://webserver2.tecgraf.puc-rio.br/iup/en/guide.html#buildlib
[iup-source]: http://webserver2.tecgraf.puc-rio.br/iup/en/svn.html
***
###OS X

You can download and install the precompiled Mac OS X binary available [here][os-x]. It appears the only download available is for OS X 10.10 64-bit.

Once you have downloaded the tarball, the installation process *should be* equivalent to Linux's starting at **Step 4**.

[os-x]: http://sourceforge.net/projects/iup/files/3.14/Other%20Libraries/

***

