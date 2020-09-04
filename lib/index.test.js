const lib = require('./index');

describe('bncsutil', () => {
    test('version', () => {
        expect(lib.version()).toBe(10300)
    });

    test('version_string', () => {
        expect(lib.version_string()).toBe('1.3.0')
    });

    test('get_exe_info', () => {
        expect(lib.get_exe_info("./mock/war3.exe")).toMatchObject( {
            exe_info: "war3.exe 12/09/16 06:05:09 515048",
            exe_version: 18547117
        })
    })
});
