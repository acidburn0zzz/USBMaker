//   Copyright © 2017-2019 Joaquim Monteiro
//
//   This file is part of USBMaker.
//
//   USBMaker is free software: you can redistribute it and/or modify
//   it under the terms of the GNU General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.
//
//   USBMaker is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License for more details.
//
//   You should have received a copy of the GNU General Public License
//   along with USBMaker.  If not, see <https://www.gnu.org/licenses/>.

use std::error::Error;
use std::fmt;
use std::io;

pub trait USBMakerError: Error {
    fn error_code(&self) -> i32;
}

#[derive(Debug)]
pub enum FormatError {
    CanceledByUser,
    CommandExecError(io::Error),
    CommandFailed(Option<i32>),
    PartitioningError(PartitioningError),
    UnknownFilesystemType(String),
    WipefsExecError(io::Error),
    WipefsFailed(Option<i32>),
}

#[derive(Debug)]
pub enum IsoError {
    CanceledByUser,
    CopyError(io::Error),
    FormatError(FormatError),
    MountError(MountError),
    PartitioningError(PartitioningError),
}

#[derive(Debug)]
pub enum MountError {
    CommandExecError(io::Error),
    CommandFailed(Option<i32>),
    TempdirCreationError(io::Error),
}

#[derive(Debug)]
pub enum PartitioningError {
    CanceledByUser,
    CommitError(io::Error),
    ConstraintError,
    DeviceOpenError(io::Error),
    DiskOpenError(io::Error),
    PartitionAddError(io::Error),
    PartitionCreateError(io::Error),
    UnknownTableType(String),
}

impl USBMakerError for FormatError {
    fn error_code(&self) -> i32 {
        match self {
            FormatError::CanceledByUser => 1,
            FormatError::CommandExecError(_) => 15,
            FormatError::CommandFailed(_) => 16,
            FormatError::PartitioningError(ref err) => err.error_code(),
            FormatError::UnknownFilesystemType(_) => 17,
            FormatError::WipefsExecError(_) => 18,
            FormatError::WipefsFailed(_) => 19,
        }
    }
}

// TODO: assign error codes
impl USBMakerError for IsoError {
    fn error_code(&self) -> i32 {
        match self {
            IsoError::CanceledByUser => 1,
            IsoError::CopyError(_) => 1,
            IsoError::FormatError(ref err) => err.error_code(),
            IsoError::PartitioningError(ref err) => err.error_code(),
            IsoError::MountError(ref err) => err.error_code(),
        }
    }
}

impl USBMakerError for MountError {
    fn error_code(&self) -> i32 {
        match self {
            MountError::CommandExecError(_) => 1,
            MountError::CommandFailed(_) => 1,
            MountError::TempdirCreationError(_) => 17,
        }
    }
}

impl USBMakerError for PartitioningError {
    fn error_code(&self) -> i32 {
        match self {
            PartitioningError::CanceledByUser => 1,
            PartitioningError::CommitError(_) => 8,
            PartitioningError::ConstraintError => 9,
            PartitioningError::DeviceOpenError(_) => 10,
            PartitioningError::DiskOpenError(_) => 11,
            PartitioningError::PartitionAddError(_) => 12,
            PartitioningError::PartitionCreateError(_) => 13,
            PartitioningError::UnknownTableType(_) => 14,
        }
    }
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormatError::CanceledByUser => write!(f, "The operation was canceled by the user"),
            FormatError::CommandExecError(ref e) => write!(f, "Failed to execute command: {}", e),
            FormatError::CommandFailed(status) => match status {
                Some(code) => write!(f, "Command exited with code: {}", code),
                None => write!(f, "Command terminated by signal"),
            },
            FormatError::PartitioningError(ref e) => e.fmt(f),
            FormatError::UnknownFilesystemType(ref s) => {
                write!(f, "Unknown filesystem type: {}", s)
            }
            FormatError::WipefsExecError(ref e) => write!(f, "Failed to execute wipefs: {}", e),
            FormatError::WipefsFailed(status) => match status {
                Some(code) => write!(f, "Wipefs exited with code: {}", code),
                None => write!(f, "Wipefs terminated by signal"),
            },
        }
    }
}

impl fmt::Display for IsoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IsoError::CanceledByUser => write!(f, "The operation was canceled by the user"),
            IsoError::CopyError(ref e) => write!(f, "Failed to copy files: {}", e),
            IsoError::FormatError(ref e) => e.fmt(f),
            IsoError::MountError(ref e) => e.fmt(f),
            IsoError::PartitioningError(ref e) => e.fmt(f),
        }
    }
}

impl fmt::Display for MountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MountError::CommandExecError(ref e) => write!(f, "Failed to execute command: {}", e),
            MountError::CommandFailed(status) => match status {
                Some(code) => write!(f, "Mount command exited with code: {}", code),
                None => write!(f, "Mount command terminated by signal"),
            },
            MountError::TempdirCreationError(ref e) => {
                write!(f, "Failed to create temporary directory: {}", e)
            }
        }
    }
}

impl fmt::Display for PartitioningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartitioningError::CanceledByUser => {
                write!(f, "The operation was canceled by the user")
            }
            PartitioningError::CommitError(ref e) => {
                write!(f, "Failed to commit changes to disk: {}", e)
            }
            PartitioningError::ConstraintError => write!(f, "Failed to get the constraint"),
            PartitioningError::DeviceOpenError(ref e) => {
                write!(f, "Failed open the target device: {}", e)
            }
            PartitioningError::DiskOpenError(ref e) => {
                write!(f, "Failed open the partition table: {}", e)
            }
            PartitioningError::PartitionAddError(ref e) => {
                write!(f, "Failed to add partition to partition table: {}", e)
            }
            PartitioningError::PartitionCreateError(ref e) => {
                write!(f, "Failed create partition in memory: {}", e)
            }
            PartitioningError::UnknownTableType(ref s) => {
                write!(f, "Unknown partition table type: {}", s)
            }
        }
    }
}

impl Error for FormatError {}
impl Error for IsoError {}
impl Error for MountError {}
impl Error for PartitioningError {}
