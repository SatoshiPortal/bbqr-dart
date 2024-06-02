# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint boltz_dart.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'bbqr_dart'
  s.version          = '0.1.0'
  s.summary          = 'A bbqr library.'
  s.description      = <<-DESC
  A bbqr library.
                       DESC
  s.homepage         = 'http://github.com/SatoshiPortal'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'SatoshiPortal' => 'ishi@satoshiportal.com' }


  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '12.0'
  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
  s.user_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
  s.swift_version = '5.0'
  s.public_header_files = 'Classes**/*.h'
  s.source_files = 'Classes/**/*'
  s.static_framework = true
  s.vendored_frameworks = "Frameworks/BbqrDart.xcframework"

  # # This will ensure the source files in Classes/ are included in the native
  # # builds of apps using this FFI plugin. Podspec does not support relative
  # # paths, so Classes contains a forwarder C file that relatively imports
  # # `../src/*` so that the C sources can be shared among all target platforms.
  # s.source           = { :path => '.' }
  # s.source_files = 'Classes/**/*'
  # s.public_header_files = 'Classes**/*.h'
  # s.dependency 'Flutter'
  # s.platform = :ios, '11.0'

  # # Flutter.framework does not contain a i386 slice.
  # s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  # s.swift_version = '5.0'
end
