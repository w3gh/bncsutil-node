const lib = require('./index');

describe('bncsutil', () => {
    test('version', () => {
        expect(lib.version()).toBe(10300)
    });

    test('version_string', () => {
        expect(lib.version_string()).toBe('1.3.0')
    })
});
