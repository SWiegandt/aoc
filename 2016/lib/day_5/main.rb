require 'digest'
require_relative '../util'

class Day5
  def initialize
    @input = 'ugkcyxxp'
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def problem_one
    (1..).lazy.map do |index|
      digest = Digest::MD5.hexdigest(@input + index.to_s)
      digest.start_with?('00000') ? digest[5] : nil
    end.reject(&:nil?).take(8).to_a.join('')
  end

  def problem_two
    password = []

    (1..).lazy.each do |index|
      digest = Digest::MD5.hexdigest(@input + index.to_s)
      password_index = digest[5]

      if digest.start_with?('00000') && password_index < '8'
        password_index = password_index.to_i
        password[password_index] = digest[6] if password[password_index].nil?
      end

      return password.join('') if (0..7).none? { |n| password[n].nil? }
    end
  end
end

Day5.new.main
