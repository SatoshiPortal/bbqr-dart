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

  s.script_phase = {
    :name => 'Build Rust library',
    # First argument is relative path to the `rust` folder, second is name of rust library
    :script => 'sh "$PODS_TARGET_SRCROOT/../cargokit/build_pod.sh" ../rust bbqr_dart',
    :execution_position => :before_compile,
    :input_files => ['${BUILT_PRODUCTS_DIR}/cargokit_phony'],
    # Let XCode know that the static library referenced in -force_load below is
    # created by this build step.
    :output_files => ["${BUILT_PRODUCTS_DIR}/libbbqr_dart.a"],
  }
  
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
    # Flutter.framework does not contain a i386 slice.
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
    'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libbbqr_dart.a',
  }

end
