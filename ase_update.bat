@echo off

set aseb=C:\Users\lollipopft\aseb
set skia=%aseb%\skia

rd /s /q %aseb%
mkdir %aseb%
mkdir %aseb%\build
mkdir %skia%

C:\Users\lollipopft\rust\projects\web\ase_update\target\debug\ase_update.exe %aseb%\aseprite.zip %skia%\skia.zip

pushd %aseb%
7z x aseprite.zip
del aseprite.zip
popd

pushd %skia%
7z x skia.zip
del skia.zip
popd

cd %aseb%\build
cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -DLAF_BACKEND=skia -DSKIA_DIR=%skia% -DSKIA_LIBRARY_DIR=%skia%\out\Release-x64 -DSKIA_LIBRARY=%skia%\out\Release-x64\skia.lib -G Ninja ..
ninja aseprite