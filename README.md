# Trojan Horse PoC in Rust

**Disclaimer**: This repository contains a Proof-of-Concept (PoC) for educational and research purposes only. Unauthorized use of this code for malicious activities is illegal and unethical. Always obtain explicit permission before testing any system.

## Overview
This repository demonstrates a simple trojan horse program written in Rust. The program disguises itself as a benign application (e.g., a calculator) while executing a hidden payload (e.g., a reverse shell). The goal is to study malware behavior, improve defensive strategies, and conduct authorized penetration testing.

## Features
- **Disguised Functionality**: Appears as a harmless calculator.
- **Hidden Payload**: Executes a background process (e.g., reverse shell) after a delay.
- **Ethical Focus**: Designed for security research and red teaming.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- A controlled testing environment (e.g., virtual machine).

## Usage
1. Clone the repository:
   ```bash
   git clone https://github.com/DDharm007/Troj-n-Horse.git
   cd Troj-n-Horse
