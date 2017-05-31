@ECHO OFF
if "%1" == "clean" (
	echo "cleaning..."
	Pushd "app\"
	cargo clean
	popd
	cargo clean
	del "static\script\app\app.js"
)
if "%1" == "rebuild" (
	echo "rebuild started."
	echo "cleaning..."
	Pushd "app\"
	cargo clean
	popd
	cargo clean
	del "static\script\app\app.js"
	echo "building..."
	Pushd "app\"
	cargo build --target=asmjs-unknown-emscripten
	popd
	cargo build
	if not exist "static\script\" mkdir "static\script\"
	if not exist "static\script\app\" mkdir "static\script\app\"
	copy "app\target\asmjs-unknown-emscripten\debug\app.js" "static\script\app\app.js"
)
if "%1" == "build" (
	echo "building..."
	Pushd "app\"
	cargo build --target=asmjs-unknown-emscripten
	popd
	cargo build
	if not exist "static\script\" mkdir "static\script\"
	if not exist "static\script\app\" mkdir "static\script\app\"
	copy "app\target\asmjs-unknown-emscripten\debug\app.js" "static\script\app\app.js"
)