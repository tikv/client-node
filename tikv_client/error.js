"use strict";


// TODO: #23 Implement UndertminedError
// 

// Create a custom error
class TransactionError extends Error {
    constructor(message) {
        super(message);
    }
}

module.exports = {
    TransactionError: TransactionError
};
