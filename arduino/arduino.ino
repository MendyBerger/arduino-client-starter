/*
 * Wasm3 - high performance WebAssembly interpreter written in C.
 * Copyright © 2020 Volodymyr Shymanskyy, Steven Massey.
 * All rights reserved.
 */

#include <wasm3.h>
#include <m3_env.h>


# define d_m3LogParse           0   // .wasm binary decoding info
# define d_m3LogModule          0   // Wasm module info
# define d_m3LogCompile         0   // wasm -> metacode generation phase
# define d_m3LogWasmStack       0   // dump the wasm stack when pushed or popped
# define d_m3LogEmit            0   // metacode-generation info
# define d_m3LogCodePages       0   // dump metacode pages when released
# define d_m3LogRuntime         0   // higher-level runtime information
# define d_m3LogNativeStack     0   // track the memory usage of the C-stack

/*
 * Configuration
 */


// #define WASM_STACK_SLOTS    2000
#define WASM_STACK_SLOTS    8000
// #define NATIVE_STACK_SIZE   (32*1024)
#define NATIVE_STACK_SIZE   (128*1024)

// For (most) devices that cannot allocate a 64KiB wasm page
// #define WASM_MEMORY_LIMIT   4096
#define WASM_MEMORY_LIMIT   15360


#include "./temp/app.wasm.h"


m3ApiRawFunction(m3_arduino_millis)
{
    m3ApiReturnType (uint32_t)

    m3ApiReturn(millis());
}

m3ApiRawFunction(m3_arduino_delay)
{
    m3ApiGetArg     (uint32_t, ms)

    // You can also trace API calls
    //Serial.print("api: delay "); Serial.println(ms);

    delay(ms);

    m3ApiSuccess();
}

// This maps pin modes from arduino_wasm_api.h
// to actual platform-specific values
uint8_t mapPinMode(uint8_t mode)
{
    switch(mode) {
        case 0: return INPUT;
        case 1: return OUTPUT;
        case 2: return INPUT_PULLUP;
    }
    return INPUT;
}

m3ApiRawFunction(m3_arduino_pinMode)
{
    m3ApiGetArg     (uint32_t, pin)
    m3ApiGetArg     (uint32_t, mode)

#if !defined(PARTICLE)
    typedef uint8_t PinMode;
#endif
    pinMode(pin, (PinMode)mapPinMode(mode));

    m3ApiSuccess();
}

m3ApiRawFunction(m3_arduino_digitalWrite)
{
    m3ApiGetArg     (uint32_t, pin)
    m3ApiGetArg     (uint32_t, value)

    digitalWrite(pin, value);

    m3ApiSuccess();
}

m3ApiRawFunction(m3_arduino_getPinLED)
{
    m3ApiReturnType (uint32_t)

    m3ApiReturn(LED_BUILTIN);
}

m3ApiRawFunction(m3_arduino_serialAvailable)
{
    m3ApiReturnType (uint32_t)

    m3ApiReturn(Serial.available());
}

m3ApiRawFunction(m3_arduino_serialRead)
{
    m3ApiReturnType (uint32_t)

    m3ApiReturn(Serial.read());
}

m3ApiRawFunction(m3_arduino_serialWrite)
{
    m3ApiGetArg  (uint8_t, c)
    Serial.print((char)c);

    m3ApiSuccess();
}

M3Result  LinkArduino  (IM3Runtime runtime)
{
    IM3Module module = runtime->modules;
    const char* arduino = "arduino";

    m3_LinkRawFunction (module, arduino, "millis",           "i()",    &m3_arduino_millis);
    m3_LinkRawFunction (module, arduino, "delay",            "v(i)",   &m3_arduino_delay);
    m3_LinkRawFunction (module, arduino, "pinMode",          "v(ii)",  &m3_arduino_pinMode);
    m3_LinkRawFunction (module, arduino, "digitalWrite",     "v(ii)",  &m3_arduino_digitalWrite);
    m3_LinkRawFunction (module, arduino, "getPinLED",        "i()",    &m3_arduino_getPinLED);
    m3_LinkRawFunction (module, arduino, "serialAvailable",  "i()",    &m3_arduino_serialAvailable);
    m3_LinkRawFunction (module, arduino, "serialRead",       "i()",    &m3_arduino_serialRead);
    m3_LinkRawFunction (module, arduino, "serialWrite",      "v(i)",   &m3_arduino_serialWrite);

    return m3Err_none;
}

/*
 * Engine start, liftoff!
 */

#define FATAL(func, msg) { Serial.print("Fatal: " func " "); Serial.println(msg); return; }

void wasm_task(void*)
{
    M3Result result = m3Err_none;

    IM3Environment env = m3_NewEnvironment ();
    if (!env) FATAL("NewEnvironment", "failed");

    IM3Runtime runtime = m3_NewRuntime (env, WASM_STACK_SLOTS, NULL);
    if (!runtime) FATAL("NewRuntime", "failed");

#ifdef WASM_MEMORY_LIMIT
    runtime->memoryLimit = WASM_MEMORY_LIMIT;
#endif

    IM3Module module;
    result = m3_ParseModule (env, &module, app_wasm, app_wasm_len);
    if (result) FATAL("ParseModule", result);

    result = m3_LoadModule (runtime, module);
    if (result) FATAL("LoadModule", result);

    result = LinkArduino (runtime);
    if (result) FATAL("LinkArduino", result);

    IM3Function f;
    result = m3_FindFunction (&f, runtime, "_start");
    if (result) FATAL("FindFunction", result);

    Serial.println("Running WebAssembly...");

    result = m3_CallV (f);

    // Arriving here means getting an error

    if (result) {
        M3ErrorInfo info;
        m3_GetErrorInfo (runtime, &info);
        Serial.print("Error: ");
        Serial.print(result);
        Serial.print(" (");
        Serial.print(info.message);
        Serial.println(")");
        if (info.file && strlen(info.file) && info.line) {
            Serial.print("At ");
            Serial.print(info.file);
            Serial.print(":");
            Serial.println(info.line);
        }
    }
}

void setup()
{
    Serial.begin(9600);

#ifdef ESP32
    // On ESP32, we can launch in a separate thread
    xTaskCreate(&wasm_task, "wasm3", NATIVE_STACK_SIZE, NULL, 5, NULL);
#else
    wasm_task(NULL);
#endif
}

void loop()
{

}
