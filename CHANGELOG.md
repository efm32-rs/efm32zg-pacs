# 0.1.0

- Generate PACs for EFM32GG and EFM32GG11B families

# 0.1.1

- Combined same family MCUs into the single crate

# 0.1.2

- Regenerated PAC crates with `svd2rust` version `0.28.0` with the enabled `atomics` feature
- Added `portable-atomic` dependency to reflect crates with the `atomics` feature generated
- Added `critical-section` dependency to reflect the latest `svd2rust` generation rules

# 0.1.3

- Regenerated PAC crates with SVD pack v4.2.0

# 0.1.4

- Fixed atomic support in PACs (`critical-section` feature introduced with proper dependency features switching)
