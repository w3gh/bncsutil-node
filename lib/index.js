// console.log(addon.create_client_public_key('jilizart', 'jilizart'));

const lib = require('../native');

/**
 * extracts number from file name "ver-IX86-1.mpq"
 * @param {string} mpqName
 * @returns {number}
 */
lib.extract_MPQ_number = (mpqName) => {
    if (!mpqName || !mpqName.length || mpqName.indexOf('.') === -1) {
        return -1
    }

    const numPos = mpqName.indexOf('.') - 1;

    return parseInt(mpqName[numPos], 16)
};

module.exports = lib;
