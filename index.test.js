const lib = require('./index');

console.log(Object.keys(lib));

describe('bncsutil', () => {
    test('version', () => {
        expect(lib.version()).toBe(10300)
    });

    test('versionString', () => {
        expect(lib.versionString()).toBe('1.3.0')
    });

    test('getExeInfo', () => {
        expect(lib.getExeInfo("./mock/war3.exe")).toMatchObject( {
            exeInfo: "war3.exe 12/09/16 06:05:09 515048",
            exeVersion: 18547117
        })
    });

    test('extractMPQNumber ver-IX86-1.mpq', () => {
        expect(lib.extractMPQNumber("ver-IX86-1.mpq")).toBe(1);
    });

    test('extractMPQNumber ver-IX86-3.mpq', () => {
        expect(lib.extractMPQNumber("ver-IX86-3.mpq")).toBe(3);
    });

    test('check_revision', () => {
        expect(lib.checkRevision(
            "B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B",
            ["./mock/war3.exe"],
            lib.extractMPQNumber("ver-IX86-1.mpq")
        )).toBe(3796461076)
    });

    test('checkRevisionFlat B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B', () => {
        // { valueString: 'A=1239576727 C=1604096186 B=4198521212 4 A=A+S B=B-C C=C^A A=A+B', file1: 'war3.exe', file2: 'Storm.dll', file3: 'game.dll', mpqNumber: 1 } 1618231241
        // { valueString: 'A=3845581634 B=880823580 C=1363937103 4 A=A-S B=B-C C=C-A A=A-B', file1: 'war3.exe', file2: 'Storm.dll', file3: 'game.dll', mpqNumber: 3 } 2863374408
        // { valueString: 'B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B', file1: 'war3.exe', file2: 'Storm.dll', file3: 'game.dll', mpqNumber: 1 } 1076278704
        expect(lib.checkRevisionFlat(
            'B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B',
            "./mock/war3.exe",
            "./mock/Storm.dll",
            "./mock/game.dll",
            lib.extractMPQNumber("ver-IX86-1.mpq"),
        )).toBe(1076278704);
    });

    test('checkRevisionFlat A=3845581634 B=880823580 C=1363937103 4 A=A-S B=B-C C=C-A A=A-B', () => {
        expect(lib.checkRevisionFlat(
            'A=3845581634 B=880823580 C=1363937103 4 A=A-S B=B-C C=C-A A=A-B',
            "./mock/war3.exe",
            "./mock/Storm.dll",
            "./mock/game.dll",
            lib.extractMPQNumber("ver-IX86-3.mpq"),
        )).toBe(2863374408);
    });

    test('checkRevisionFlat A=1239576727 C=1604096186 B=4198521212 4 A=A+S B=B-C C=C^A A=A+B', () => {
        expect(lib.checkRevisionFlat(
            'A=1239576727 C=1604096186 B=4198521212 4 A=A+S B=B-C C=C^A A=A+B',
            "./mock/1.28.5/war3.exe",
            "./mock/1.28.5/Storm.dll",
            "./mock/1.28.5/game.dll",
            lib.extractMPQNumber("ver-IX86-1.mpq"),
        )).toBe(1618231241);
    });
});
