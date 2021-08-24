fn main() {
    windows::build! {
        Windows::Win32::Storage::FileSystem::GetFileVersionInfoSizeA,
        Windows::Win32::Storage::FileSystem::GetFileVersionInfoA,
        Windows::Win32::Storage::FileSystem::VerQueryValueA,
        Windows::Win32::Storage::FileSystem::VS_FIXEDFILEINFO,
        Windows::Win32::System::LibraryLoader::GetModuleFileNameA,
        Windows::Win32::Foundation::PSTR,
        Windows::Win32::Foundation::HINSTANCE,
    };
}
