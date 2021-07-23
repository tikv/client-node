"use strict";



// Create a custom error
class OperationAfterCommitError extends Error {
    constructor(message) {
        super(message);
        this.message = "Cannot read or write data after any attempt to commit or roll back the transaction";
    }
}

module.exports = {
    OperationAfterCommitError: OperationAfterCommitError
  };
  