const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'bncsutil-node' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `bncsutil-node.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `bncsutil-node-[PLATFORM]`
 */
const lib = loadBinding(__dirname, 'bncsutil-node', 'bncsutil-node')

/**
 * extracts number from file name "ver-IX86-1.mpq"
 * @param {string} mpqName
 * @returns {number}
 */
lib.extractMPQNumber = (mpqName) => {
    if (!mpqName || !mpqName.length || mpqName.indexOf('.') === -1) {
        return -1
    }

    const numPos = mpqName.indexOf('.') - 1;

    return parseInt(mpqName[numPos], 16)
};

module.exports = lib;
