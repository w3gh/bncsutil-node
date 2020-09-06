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
    });

    test('extract_MPQ_number ver-IX86-1.mpq', () => {
        expect(lib.extract_MPQ_number("ver-IX86-1.mpq")).toBe(1);
    });

    test('extract_MPQ_number ver-IX86-3.mpq', () => {
        expect(lib.extract_MPQ_number("ver-IX86-3.mpq")).toBe(3);
    });

    test('check_revision', () => {
        expect(lib.check_revision(
            "B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B",
            ["./mock/war3.exe"],
            lib.extract_MPQ_number("ver-IX86-1.mpq")
        )).toBe(3796461076)
    });

    test('check_revision_flat B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B', () => {
        // { valueString: 'A=1239576727 C=1604096186 B=4198521212 4 A=A+S B=B-C C=C^A A=A+B', file1: 'war3.exe', file2: 'Storm.dll', file3: 'game.dll', mpqNumber: 1 } 1618231241
        // { valueString: 'A=3845581634 B=880823580 C=1363937103 4 A=A-S B=B-C C=C-A A=A-B', file1: 'war3.exe', file2: 'Storm.dll', file3: 'game.dll', mpqNumber: 3 } 2863374408
        // { valueString: 'B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B', file1: 'war3.exe', file2: 'Storm.dll', file3: 'game.dll', mpqNumber: 1 } 1076278704
        expect(lib.check_revision_flat(
            'B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B',
            "./mock/war3.exe",
            "./mock/Storm.dll",
            "./mock/game.dll",
            lib.extract_MPQ_number("ver-IX86-1.mpq"),
        )).toBe(1076278704);
    });

    test('check_revision_flat A=3845581634 B=880823580 C=1363937103 4 A=A-S B=B-C C=C-A A=A-B', () => {
        expect(lib.check_revision_flat(
            'A=3845581634 B=880823580 C=1363937103 4 A=A-S B=B-C C=C-A A=A-B',
            "./mock/war3.exe",
            "./mock/Storm.dll",
            "./mock/game.dll",
            lib.extract_MPQ_number("ver-IX86-3.mpq"),
        )).toBe(2863374408);
    });

    test('check_revision_flat A=1239576727 C=1604096186 B=4198521212 4 A=A+S B=B-C C=C^A A=A+B', () => {
        expect(lib.check_revision_flat(
            'A=1239576727 C=1604096186 B=4198521212 4 A=A+S B=B-C C=C^A A=A+B',
            "./mock/1.28.5/war3.exe",
            "./mock/1.28.5/Storm.dll",
            "./mock/1.28.5/game.dll",
            lib.extract_MPQ_number("ver-IX86-1.mpq"),
        )).toBe(1618231241);
    });
});
