"use strict";


// TODO: #23 Implement UndertminedError
// 

// Create a custom error
class OperationAfterCommitError extends Error {
    constructor(message) {
        super(message);
    }
}
class UndertminedError extends Error {
    constructor(message) {
        super(message);
    }
}
class WriteConflictError extends Error {
    constructor(message) {
        super(message);
    }
}
class AlreadyExistError extends Error {
    constructor(message) {
        super(message);
    }
}
class DeadlockError extends Error {
    constructor(message) {
        super(message);
    }
}

module.exports = {
    OperationAfterCommitError: OperationAfterCommitError,
    UndertminedError: UndertminedError,
    WriteConflictError: WriteConflictError,
    AlreadyExistError: AlreadyExistError,
    DeadlockError: DeadlockError,
};
